import os
import json

def main():
    # Read templates.json to get list of template IDs
    with open('templates.json', 'r') as f:
        templates = json.load(f)

    template_ids = [t['id'] for t in templates]

    # Start generating embedded.rs
    output = []
    output.append('// Embed templates.json')
    output.append('pub const TEMPLATES_JSON: &str = include_str!("../templates.json");')
    output.append('')
    output.append('// Embed font data')
    output.append('pub const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/TitilliumWeb-Black.ttf");')
    output.append('')
    output.append('// Function to get template config')
    output.append('pub fn get_template_config(template_id: &str) -> Option<&\'static str> {')
    output.append('    match template_id {')

    # Add template configs
    for template_id in template_ids:
        config_path = f'assets/templates/{template_id}/config.yml'
        if os.path.exists(config_path):
            output.append(f'        "{template_id}" => Some(include_str!("../assets/templates/{template_id}/config.yml")),')

    output.append('        _ => None')
    output.append('    }')
    output.append('}')
    output.append('')

    # Add template images
    output.append('// Function to get template image')
    output.append('pub fn get_template_image(template_id: &str, image_name: &str) -> Option<&\'static [u8]> {')
    output.append('    match (template_id, image_name) {')

    for template_id in template_ids:
        template_dir = f'assets/templates/{template_id}'
        if os.path.exists(template_dir):
            for file in os.listdir(template_dir):
                if file.endswith(('.jpg', '.png', '.gif')):
                    output.append(f'        ("{template_id}", "{file}") => Some(include_bytes!("../assets/templates/{template_id}/{file}")),')

    output.append('        _ => None')
    output.append('    }')
    output.append('}')

    # Write output
    with open('src/embedded.rs', 'w') as f:
        f.write('\n'.join(output))

if __name__ == '__main__':
    main()
