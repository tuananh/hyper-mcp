# gitlab

A plugin that implements GitLab operations including issue management, file handling, branch management, and snippet operations.

## Configuration

The plugin requires the following configuration:

- `GITLAB_TOKEN`: (Required) Your GitLab personal access token
- `GITLAB_URL`: (Optional) Your GitLab instance URL. Defaults to `https://gitlab.com/api/v4`

## Usage

```json
{
  "plugins": [
    {
      "name": "gitlab",
      "path": "oci://ghcr.io/tuananh/gitlab-plugin:latest",
      "runtime_config": {
        "allowed_hosts": ["gitlab.com"], // Your GitLab host
        "env_vars": {
          "GITLAB_TOKEN": "your-gitlab-token",
          "GITLAB_URL": "https://gitlab.com/api/v4"  // Optional, defaults to GitLab.com
        }
      }
    }
  ]
}
```

## Available Operations

### Issues
- [x] `gl_create_issue`: Create a new issue
- [x] `gl_get_issue`: Get issue details
- [x] `gl_update_issue`: Update an existing issue
- [x] `gl_add_issue_comment`: Add a comment to an issue
- [x] `gl_list_issues`: List issues for a project in GitLab. Supports filtering by state and labels.

### Files
- [x] `gl_get_file_contents`: Get file contents
- [x] `gl_create_or_update_file`: Create or update a file
- [x] `gl_delete_file`: Delete a file from the repository
- [ ] `gl_push_files`: Push multiple files

### Branches and Merge Requests
- [x] `gl_create_branch`: Create a new branch
- [x] `gl_list_branches`: List all branches in a GitLab project
- [x] `gl_create_merge_request`: Create a merge request
- [x] `gl_update_merge_request`: Update an existing merge request in a GitLab project.
- [x] `gl_get_merge_request`: Get details of a specific merge request in a GitLab project.

### Snippets
- [x] `gl_create_snippet`: Create a new snippet
- [x] `gl_update_snippet`: Update an existing snippet
- [x] `gl_get_snippet`: Get snippet details
- [x] `gl_delete_snippet`: Delete a snippet

### Repository
- [x] `gl_get_repo_tree`: Get the list of files and directories in a project repository. Handles pagination internally.
- [x] `gl_get_repo_members`: Get a list of members for a GitLab project. Supports fetching direct or inherited members and filtering by query. Handles pagination internally.
