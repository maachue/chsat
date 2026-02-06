// WARN: My english isn't good enough for writing spec
// If you have understand problem, open a issue with tag `docs`

fn _schema() {
    // run a command
    exec(["paru", "-S", "calibre-bin"]);
    // or
    sh("paru -S calibre-bin"); // using shell


    // `sudo` block if required
    sudo {
        exec(["pacman", "-S", "neovim"]);
    }

    // some distro I will auto expose package manager:
    // - CachyOS / ArchLinux / EndeavourOS
    // - Fedora / Nobara

    install(["neovim", "niri"]);         // automatically map to exec(["pacman", "-S", "niri"]) in `sudo`
    remove(["mako"]);                    // automatically map to exec(["pacman", "-Rns", "mako"]) in `sudo`
    update_meta();                   // automatically map to exec(["pacman", "-Syy"]) in `sudo`
    update_pkgs();                       // automatically map to exec(["pacman", "-Syu"]) in `sudo`

    // WARN: DO LIKE THIS WILL MAKE THE SCIRPT ALWAYS FAILED
    ```rhai
    sudo {
        install(["neovim"])
    }
    ```
    // Why? double `sudo`.
    // Always will cause the script failed when dected double `sudo` case. Type: `DoubleSudoError` (fail-fast)

    // WARN: By default, `exec()`, `install()`, `remove()`, `update_pkgs()`, `update_meta()` can be error. But write like this will ignore that.
    // Also, the program will stopped when a methods return error. aka fail-fast

    // if you using a unsupported distro and still want auto-mapping, you needed expose these var
    // Debian-based for example: (this is global)
    chsat.packageManager.binaryName = "apt"; // the binary path will automatically generate. If you want, you can expose the binary path.
    chsat.packageManager.flags       = map_flags( // needed `map_flags` to mapping.
                        install=["install"],
                        remove=["remove"],
                        update_pkgs=["upgrade"],
                        update_meta=["update"]
    );
    chsat.packageManager.requiredSudo = true; // if this field == true, it always added the auto-mapping funcs the `sudo` (can cause `DoubleSudoError`)
    chsat.packageManager.usage = "{{binary_name}} {{flags}} {{{opts}} {{pkgs}}";
    // default: "{{binary_name}} {{flags}} {{pkgs}} {{opts}}"
    // `opts` field: that is no_confirm. Nothing special.

    // Fedora-based & Arch-based: "{{binary_name}} {{flags}} {{pkgs}} {{opts}}"

    // NOTE: actually `pkgs` is a flags. So you can do something like this (but need carefully)
    install(["neovim", "matugen", "--needed"]);
    // NOTE: But I recommended using this: (also, this is how you can override once by once)
    chsat.packageManager.flags.installFlags = ["-S", "--needed"]; // `pacman` is very pleasant

    // Path resolver
    let wallsec = path("~/.local/bin/walsec"); // required `path()` to auto expose the path
    // NOTE: the `path` type can display by default not like PathBuf in Rust
    print(wallsec.exists());                   // true or false?
    print(wallsec.join("doesn't exists"));     // add path, not check exists by default
    print(wallsec.is_dir());                   // false or true?
    print(wallsec.parent());                   // return the parent folder. In this case is `~/.local/bin`
    print(wallsec.normalize());                // resolve everything like `.`, `..`, `\\`or `//`
    print(wallsec.to_string());                 // for special case (kinda), expose for fun

    // Run a script
    exec(wallsec); // path will auto mapping

    // Error handling? yup, Rhai has it
    try {
        exec(wallsec);
    } catch (err) {
        print(err) // or custom error msg. The default is very minimal
    }

    // this exec will ignore the error and continue the script
    exec_ignore(wallsec); // do not fail-fast when the program failed

    // You want to check the program already in path? Yeah, I have a method
    if checker("flatpak") {
        flatpak_install(["com.discordapp.Discord", "com.github.tchx84.Flatseal"]);
    }

    // somethings I will expose by default for you <3
    // (use: `chsat data` for more infomation)
    print(chsat.os); // "linux"
    print(chsat.osRelease.id);         // "cachyos"
    print(chsat.osRelease.name);       // "CachyOS Linux"
    print(chsat.osRelease.prettyName) // "CachyOS"
}

// hacks
fn flatpak_install(pkgs) {
    exec(["flatpak", "install"] += pkgs);
}

// let's go for real-usage

fn setup() {
    exec(["rustup", "install", "stable"]);
    exec(["nvim", "--headless", '"+Lazy! sync"', "+qa"]);
}

if chsat.os == "linux" {
    try {
        let distro = chsat.osRelease.id;

        if distro == "cachyos" || distro == "endeavouros" || distro == "archlinux" {
            install(["neovim", "flatpak", "rustup", "--needed"])

            if distro == "cachyos" {
                install(["paru"]);
            }

            setup();
        } else {
            print("unsupported distro");
        }
    catch (err) {
        print(err);
    }
} else {
    print("unsupported OS")
}

// // feature proof
// include("$DOTFILES/something.rhai");
