import subprocess
import yaml
import threading

DEFAULT_L = 10
DEFAULT_PRECISION = "f64"
DEFAULT_N_THREADS = 11
DEFAULT_N_TRIES = 3


def stream_reader(pipe, file_obj, print_prefix=""):
    for line in iter(pipe.readline, ""):
        if not line:
            break
        file_obj.write(line)
        file_obj.flush()
        print(print_prefix + line, end="")
    pipe.close()


def run_command(args, stdout_path, stderr_path):
    print("Executing:", " ".join(args))

    with open(stdout_path, "a", encoding="utf-8") as f_out, open(
        stderr_path, "a", encoding="utf-8"
    ) as f_err:

        proc = subprocess.Popen(
            args,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
            universal_newlines=True,
        )

        t_out = threading.Thread(target=stream_reader, args=(proc.stdout, f_out))
        t_err = threading.Thread(
            target=stream_reader, args=(proc.stderr, f_err, "ERR: ")
        )

        t_out.start()
        t_err.start()

        t_out.join()
        t_err.join()

        retcode = proc.wait()

    if retcode != 0:
        print(f"\nErrors found. See log: {stderr_path}")
        raise subprocess.CalledProcessError(retcode, args)


def compile(**kwargs):
    build_config = {
        "L": kwargs.get("L", DEFAULT_N_THREADS),
        "precision": kwargs.get("precision", DEFAULT_PRECISION),
        "n_threads": kwargs.get("n_threads", DEFAULT_N_THREADS),
        "n_tries": kwargs.get("n_tries", DEFAULT_N_TRIES),
    }

    print("Building with config:", build_config)
    yaml_path = r"..\config.yaml"
    with open(yaml_path, "w") as file:
        yaml.dump(build_config, file, default_flow_style=False)

    args = ["cargo", "build", "--release"]
    run_command(args, "log.out", "log.err")


def run(dist, param, export_mode="exportisosurface", surfval=0.0, outdir="."):
    args = [
        r"..\target\release\solver.exe",
        "--dist",
        dist,
        "--param",
        str(param),
        "--export",
        export_mode,
        "--surfval",
        str(surfval),
        "--outdir",
        outdir,
    ]

    run_command(args, "log.out", "log.err")


surfval = 1 - 1e-3
dist = "inverse"
Ls = [10, 15, 20, 25, 35]
n_tries = 200

build_cases = [
    (200, "f256"),
    (100, "f256"),
    (30, "f256"),
    (15, "f64"),
    (7, "f64"),
]

for L in Ls:
    for param, prec in build_cases:
        compile(L=L, n_tries=n_tries, precision=prec)
        run(
            dist,
            param,
            surfval=surfval,
            outdir="./arrays",
            export_mode="exportarrays",
        )
