#!/usr/bin/env bash
set -euo pipefail

while IFS= read -r -d '' tracked_path; do
    case "${tracked_path}" in
        *.zip|*.tar|*.tar.gz|*.tgz)
            printf 'forbidden tracked archive found: %s\n' "${tracked_path}" >&2
            exit 1
            ;;
    esac
done < <(git ls-files -z)

if [[ -d catalog/imports ]]; then
    printf 'forbidden directory found: catalog/imports/\n' >&2
    exit 1
fi
