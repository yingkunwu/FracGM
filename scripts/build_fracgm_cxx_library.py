import update_fracgm_cxx_include
import utils


def build_fracgm_cxx_library():
    update_fracgm_cxx_include.update_fracgm_cxx_include()

    # cargo build --release
    fracgm_cxx_path = utils.get_project_root() / "fracgm-cxx"

    utils.run_command("cargo build --release", cwd=fracgm_cxx_path)


if __name__ == "__main__":
    build_fracgm_cxx_library()
    utils.get_console().print("[bold green]Done.")
