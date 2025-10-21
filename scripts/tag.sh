#!/usr/bin/env bash
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

usage() {
    cat << EOF
Usage: $0 [options]

Options:
    major       Update major version (x.0.0)
    minor       Update minor version (0.x.0)
    patch       Update patch version (0.0.x) [default]
    -h, --help  Show this help message
EOF
}

error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

get_current_version() {
    local cargo_toml="Cargo.toml"
    if [[ ! -f "$cargo_toml" ]]; then
        error "Cargo.toml not found"
    fi

    grep -E '^version = ' "$cargo_toml" | head -1 | sed -E 's/version = "(.*)"/\1/'
}

bump_version() {
    local current_version=$1
    local bump_type=$2

    local major minor patch
    IFS='.' read -r major minor patch <<< "$current_version"

    case "$bump_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            error "Invalid version type: $bump_type"
            ;;
    esac

    echo "${major}.${minor}.${patch}"
}

update_cargo_toml() {
    local new_version=$1
    local cargo_toml="Cargo.toml"

    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_toml"
    else
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_toml"
    fi
}


check_git_status() {
    if ! git diff-index --quiet HEAD --; then
        error "Uncommitted changes found. Please commit them first."
    fi
}

get_main_branch() {
    if git rev-parse --verify main >/dev/null 2>&1; then
        echo "main"
    elif git rev-parse --verify master >/dev/null 2>&1; then
        echo "master"
    else
        error "Neither 'main' nor 'master' branch found"
    fi
}

create_tag() {
    local version=$1
    local tag_name="v$version"
    local current_branch=$(git rev-parse --abbrev-ref HEAD)
    local main_branch=$(get_main_branch)

    if git rev-parse "$tag_name" >/dev/null 2>&1; then
        error "Tag '$tag_name' already exists"
    fi

    git add Cargo.toml Cargo.lock

    git commit -m "chore: bump version to $version"
    success "Changes committed"

    # 現在のブランチがmain/masterでない場合、PRワークフローを実行
    if [[ "$current_branch" != "$main_branch" ]]; then
        info "Pushing branch '$current_branch'..."
        git push origin "$current_branch"
        success "Branch pushed"

        info "Merging PR with gh command..."
        if gh pr merge --auto --squash --delete-branch 2>/dev/null; then
            success "PR merged and branch deleted"
        else
            warning "Failed to auto-merge. Attempting manual merge..."
            if gh pr merge --squash --delete-branch; then
                success "PR merged and branch deleted"
            else
                error "Failed to merge PR. Please merge manually and run this script again."
            fi
        fi

        info "Switching to $main_branch branch..."
        git checkout "$main_branch"
        success "Switched to $main_branch"

        info "Pulling latest changes..."
        git pull origin "$main_branch"
        success "Pulled latest changes"
    fi

    git tag -a "$tag_name" -m "Release $version"
    success "Tag '$tag_name' created"

    info "Pushing tag to origin..."
    git push origin "$tag_name"
    success "Tag '$tag_name' pushed"
}

main() {
    local bump_type="patch"

    if [[ $# -gt 0 ]]; then
        case "$1" in
            -h|--help)
                usage
                exit 0
                ;;
            major|minor|patch)
                bump_type=$1
                ;;
            *)
                error "Invalid argument: $1\nPlease use -h or --help to see the usage"
                ;;
        esac
    fi

    info "Version update type: $bump_type"

    check_git_status

    local current_version
    current_version=$(get_current_version)
    info "Current version: $current_version"

    local new_version
    new_version=$(bump_version "$current_version" "$bump_type")
    info "New version: $new_version"

    echo ""
    read -p "Update version from $current_version to $new_version? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        error "Processing interrupted"
    fi

    update_cargo_toml "$new_version"
    success "Cargo.toml updated"

    info "Updating Cargo.lock..."
    cargo update --quiet
    success "Cargo.lock updated"

    create_tag "$new_version"

    echo ""
    success "Done! 🎉"
}

main "$@"
