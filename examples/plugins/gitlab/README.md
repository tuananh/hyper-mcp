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

### Files
- [x] `gl_get_file_contents`: Get file contents
- [x] `gl_create_or_update_file`: Create or update a file
- [ ] `gl_push_files`: Push multiple files

### Branches and Merge Requests
- [x] `gl_create_branch`: Create a new branch
- [x] `gl_create_merge_request`: Create a merge request

### Snippets
- [x] `gl_create_snippet`: Create a new snippet
- [x] `gl_update_snippet`: Update an existing snippet
- [x] `gl_get_snippet`: Get snippet details
- [x] `gl_delete_snippet`: Delete a snippet
