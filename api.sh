#!/bin/sh
atac -d . request send todos/healthcheck --env example --status-code --hide-content
atac -d . request send todos/create --env example --status-code --hide-content
atac -d . request send todos/get --env example --status-code --hide-content
atac -d . request send todos/update --env example --status-code --hide-content
atac -d . request send todos/create --env example --status-code --hide-content
atac -d . request send todos/delete --env example --status-code --hide-content
atac -d . request send todos/list --env example --status-code
