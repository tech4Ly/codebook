function getLanguageFromFileName(fileName: string): string {
  // Extract the extension from the filename
  const extension = fileName.split('.').pop()?.toLowerCase() || '';

  // Map of file extensions to language IDs
  const extensionToLanguage: { [key: string]: string } = {
    // Programming Languages
    'ts': 'typescript',
    'js': 'javascript',
    'py': 'python',
    'java': 'java',
    'cpp': 'cpp',
    'c': 'c',
    'cs': 'csharp',
    'rb': 'ruby',
    'php': 'php',
    'go': 'go',
    'rs': 'rust',
    'swift': 'swift',
    'kt': 'kotlin',

    // Web Technologies
    'html': 'html',
    'htm': 'html',
    'css': 'css',
    'scss': 'scss',
    'sass': 'sass',
    'jsx': 'javascriptreact',
    'tsx': 'typescriptreact',
    'vue': 'vue',

    // Data Formats
    'json': 'json',
    'xml': 'xml',
    'yaml': 'yaml',
    'yml': 'yaml',
    'md': 'markdown',

    // Shell Scripts
    'sh': 'shell',
    'bash': 'shell',
    'ps1': 'powershell',

    // Other
    'sql': 'sql',
    'dockerfile': 'dockerfile',
    'txt': 'plaintext'
  };

  // Return the language ID if found, otherwise return 'plaintext'
  return extensionToLanguage[extension] || 'plaintext';
}

export {getLanguageFromFileName}
