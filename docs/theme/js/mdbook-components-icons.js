/* mdbook-components: Devicon font adapter and generic interface icons.
   Devicon 2.17.0 CSS must load before this script; no runtime SVG requests.
   To add an icon: add its official Devicon name to branded below.
*/
const MDCIcons = (() => {
  "use strict";
  const NS = "http://www.w3.org/2000/svg";
  const FILE = [
    [
      "path",
      { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" },
    ],
    ["path", { d: "M14 2v6h6" }],
  ];
  const shapes = {
    folder: [
      [
        "path",
        {
          d: "M3 7a2 2 0 0 1 2-2h5l2 2h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
        },
      ],
    ],
    file: FILE,
    code: [["path", { d: "m8 8-4 4 4 4m8-8 4 4-4 4" }]],
    settings: [
      ["circle", { cx: "12", cy: "12", r: "3" }],
      [
        "path",
        {
          d: "M12 2v3m0 14v3M2 12h3m14 0h3M5 5l2 2m10 10 2 2M19 5l-2 2M7 17l-2 2",
        },
      ],
    ],
    image: [
      ["rect", { x: "3", y: "3", width: "18", height: "18", rx: "2" }],
      ["circle", { cx: "8.5", cy: "8.5", r: "1.5" }],
      ["path", { d: "m21 15-5-5L5 21" }],
    ],
    markdown: [
      ["path", { d: "M3 6h18v12H3zM6 15V9l3 3 3-3v6m3-3 2 2 2-2m-2 2V9" }],
    ],
    rust: [
      ["circle", { cx: "12", cy: "12", r: "9" }],
      ["path", { d: "M8 17V7h5a3 3 0 0 1 0 6H8m5 0 4 4" }],
    ],
    javascript: [
      ["rect", { x: "3", y: "3", width: "18", height: "18", rx: "2" }],
      [
        "path",
        { d: "M10 8v8c0 2-3 2-4 0m8-1c1 2 4 2 4 0 0-2-4-2-4-4 0-2 3-3 4-1" },
      ],
    ],
    database: [
      ["ellipse", { cx: "12", cy: "5", rx: "9", ry: "3" }],
      ["path", { d: "M3 5v14c0 4 18 4 18 0V5M3 12c0 4 18 4 18 0" }],
    ],
    text: [...FILE, ["path", { d: "M8 13h8M8 17h8" }]],
    star: [
      [
        "path",
        { d: "m12 2 3.1 6.3 7 .9-5.1 5 .9 7-6.9-3.5L5.1 21l.9-7L1 9l7-.9z" },
      ],
    ],
    moon: [["path", { d: "M20.9 13A9 9 0 0 1 11 3.1 9 9 0 1 0 20.9 13Z" }]],
    terminal: [
      ["rect", { x: "3", y: "4", width: "18", height: "16", rx: "2" }],
      ["path", { d: "m7 9 3 3-3 3m5 0h5" }],
    ],
    monitor: [["path", { d: "M3 4h18v13H3zM8 21h8m-4-4v4" }]],
    package: [
      ["path", { d: "M12 2 3 7l9 5 9-5-9-5zM3 7v10l9 5 9-5V7M12 12v10" }],
    ],
    linux: [
      ["path", { d: "M8 18a4 4 0 0 1 8 0M8 8a4 4 0 1 1 8 0m-8 0v6m8-6v6" }],
    ],
    apple: [
      [
        "path",
        {
          d: "M12 6c2-3 5-3 6-2-2 2-4 3-6 2ZM12 8c-2-2-7-1-7 5s4 9 7 7c3 2 7-2 7-7s-5-7-7-5",
        },
      ],
    ],
    book: [
      [
        "path",
        {
          d: "M12 6C9 4 5 4 2 5v14c3-1 7-1 10 1m0-14c3-2 7-2 10-1v14c-3-1-7-1-10 1V6",
        },
      ],
    ],
    check: [["path", { d: "M4 12l5 5L20 6" }]],
  };
  // Supported Devicon font glyphs, rendered in the current mdBook foreground colour.
  // Unknown brands fall back to generic interface icons; no SVG assets are required.
  const branded = new Set([
    "aarch64",
    "android",
    "androidstudio",
    "apple",
    "arduino",
    "bash",
    "c",
    "clion",
    "cmake",
    "cplusplus",
    "csharp",
    "css3",
    "debian",
    "docker",
    "gcc",
    "git",
    "github",
    "gitlab",
    "go",
    "gradle",
    "html5",
    "intellij",
    "java",
    "javascript",
    "jetbrains",
    "json",
    "jupyter",
    "kalilinux",
    "kotlin",
    "latex",
    "linux",
    "linuxmint",
    "lua",
    "mariadb",
    "markdown",
    "matlab",
    "microsoftsqlserver",
    "mongodb",
    "mysql",
    "nano",
    "neovim",
    "nginx",
    "nixos",
    "nodejs",
    "nodewebkit",
    "npm",
    "openapi",
    "php",
    "postgresql",
    "powershell",
    "pycharm",
    "pypi",
    "python",
    "pytorch",
    "raspberrypi",
    "ruby",
    "rust",
    "sqlite",
    "ssh",
    "swift",
    "tmux",
    "toml",
    "typescript",
    "ubuntu",
    "vim",
    "visualstudio",
    "vscode",
    "windows11",
    "xml",
    "yaml",
    "zsh",
  ]);
  const aliases = {
    cpp: "cplusplus",
    "c++": "cplusplus",
    cc: "cplusplus",
    cxx: "cplusplus",
    nix: "nixos",
    nixpkgs: "nixos",
    flake: "nixos",
    kt: "kotlin",
    kts: "kotlin",
    shell: "bash",
    sh: "bash",
    fish: "bash",
    ps: "powershell",
    pwsh: "powershell",
    ps1: "powershell",
    py: "python",
    python3: "python",
    pip: "pypi",
    pip3: "pypi",
    windows: "windows11",
    kali: "kalilinux",
    node: "nodejs",
    vscode: "vscode",
    "vs-code": "vscode",
    "visual-studio-code": "vscode",
    postgres: "postgresql",
    postgresql: "postgresql",
    mongo: "mongodb",
    arduinoide: "arduino",
    sqlserver: "microsoftsqlserver",
    sql: "database",
    cargo: "package",
    make: "settings",
    makefile: "settings",
    claude: "file",
    ollama: "file",
    qwen: "file",
    herdr: "file",
    npm: "npm",
    ts: "typescript",
    js: "javascript",
    html: "html5",
    css: "css3",
    rs: "rust",
    md: "markdown",
    mdx: "markdown",
    yml: "yaml",
    sqlite3: "sqlite",
    psh: "powershell",
    nvim: "neovim",
    latex: "latex",
    config: "settings",
    gear: "settings",
    success: "check",
    macos: "apple",
  };
  function resolve(name) {
    const key = String(name || "")
      .trim()
      .toLowerCase();
    const normalized = aliases[key] || key;
    return branded.has(normalized) || Object.hasOwn(shapes, normalized)
      ? normalized
      : null;
  }
  function shape(svg, tag, attrs) {
    const el = document.createElementNS(NS, tag);
    for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
    svg.append(el);
    return el;
  }
  function create(name, className = "") {
    const kind = resolve(name);
    if (!kind) return null;
    if (branded.has(kind)) {
      // Devicon font glyphs preserve genuine logos and inherit mdBook colours.
      // No SVG masks, embedded images, or per-icon runtime requests.
      const glyph = document.createElement("i");
      glyph.className = `mdc-devicon devicon-${kind}-plain`;
      if (className)
        glyph.classList.add(...className.split(/\s+/).filter(Boolean));
      glyph.dataset.icon = kind;
      glyph.setAttribute("aria-hidden", "true");
      return glyph;
    }
    const svg = document.createElementNS(NS, "svg");
    if (className) svg.setAttribute("class", className);
    svg.setAttribute("viewBox", "0 0 24 24");
    svg.setAttribute("fill", "none");
    svg.setAttribute("stroke", "currentColor");
    svg.setAttribute("stroke-width", "1.7");
    svg.setAttribute("stroke-linecap", "round");
    svg.setAttribute("stroke-linejoin", "round");
    svg.setAttribute("aria-hidden", "true");
    svg.setAttribute("focusable", "false");
    svg.dataset.icon = kind;
    for (const [tag, attrs] of shapes[kind]) shape(svg, tag, attrs);
    return svg;
  }
  function classify(filename, hasChildren = false) {
    const name = String(filename || "")
      .trim()
      .replace(/\/$/, "");
    const lower = name.toLowerCase();
    if (hasChildren || String(filename).trim().endsWith("/")) return "folder";
    if (lower === "cargo.toml" || lower === "cargo.lock") return "package"; // Cargo absent from Devicon
    if (/^(requirements(?:-[^/]*)?\.txt|pipfile(?:\.lock)?)$/.test(lower))
      return "pypi";
    if (/^(pyproject\.toml|setup\.py|setup\.cfg|\.python-version)$/.test(lower))
      return "python";
    if (/(?:\.nix$|^flake\.lock$)/.test(lower)) return "nixos";
    if (/^(makefile|gnumakefile)$/.test(lower)) return "settings"; // Make absent from Devicon
    if (/^(dockerfile|containerfile)$/.test(lower)) return "docker";
    if (/^(\.gitignore|\.gitattributes|\.gitmodules)$/.test(lower))
      return "git";
    if (/^(\.editorconfig)$/.test(lower)) return "settings";
    if (/^(\.zshrc|\.zprofile|\.zshenv)$/.test(lower)) return "zsh";
    if (/^(\.bashrc|\.profile|\.bash_profile)$/.test(lower)) return "bash";
    if (/\.(cpp|cc|cxx|hpp|hh|hxx|ipp)$/.test(lower)) return "cplusplus";
    if (/\.(c|h)$/.test(lower)) return "c";
    if (/\.rs$/.test(lower)) return "rust";
    if (/\.(tsx?|mts|cts)$/.test(lower)) return "typescript";
    if (/\.(jsx?|mjs|cjs)$/.test(lower)) return "javascript";
    if (/\.(kt|kts)$/.test(lower)) return "kotlin";
    if (/\.java$/.test(lower)) return "java";
    if (/\.(xml|xsd|xsl|xslt|svg)$/.test(lower))
      return lower.endsWith(".svg") ? "image" : "xml";
    if (/\.zsh$/.test(lower)) return "zsh";
    if (/\.(sh|bash)$/.test(lower)) return "bash";
    if (/\.(ps1|psm1|psd1)$/.test(lower)) return "powershell";
    if (/\.pyw?$/.test(lower)) return "python";
    if (/\.toml$/.test(lower)) return "toml";
    if (/^(package(-lock)?\.json)$/.test(lower)) return "npm";
    if (/\.jsonc?$/.test(lower)) return "json";
    if (/\.(ya?ml)$/.test(lower)) return "yaml";
    if (/\.(md|mdx|markdown)$/.test(lower)) return "markdown";
    if (/\.(png|jpe?g|gif|webp|ico|bmp|avif)$/.test(lower)) return "image";
    if (/\.sql$/.test(lower)) return "database";
    if (/\.(db|sqlite|sqlite3)$/.test(lower)) return "sqlite";
    if (/\.(txt|log|csv)$/.test(lower)) return "text";
    if (/\.go$/.test(lower)) return "go";
    if (/\.(tex|latex)$/.test(lower)) return "latex";
    if (/^(pom\.xml)$/.test(lower)) return "java";
    if (/\.html?$/.test(lower)) return "html5";
    if (/\.css$/.test(lower)) return "css3";
    if (/\.(php|swift|rb)$/.test(lower))
      return lower.split(".").pop() === "rb" ? "ruby" : lower.split(".").pop();
    return "file";
  }
  return Object.freeze({
    create,
    resolve,
    classify,
    names: Object.freeze([...Object.keys(shapes), ...branded]),
  });
})();
window.MDCIcons = MDCIcons;
