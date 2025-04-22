using Plots
using Plots.PlotMeasures
using LaTeXStrings

function load_surface(file, L, n_surface)
    surface = zeros(Float64, L, L)

    open(file) do f
        n_line = 0
        line = ""
        while n_line < n_surface
            line = readline(f)
            n_line += 1
        end
        coords = split(line, " ")
        if length(coords) % 2 != 0
            println("Invalid line: $line")
            return nothing
        end
        cols = 2 .+ parse.(Int64, coords[2:2:end])
        rows = 1 .+ parse.(Int64, coords[1:2:end])
        for i in eachindex(cols)
            surface[rows[i], cols[i]] = 1
        end
    end
    return surface
end

function find_file(srcdir, L, dist, param)
    files = readdir(srcdir)
    for file in files
        m = match(pattern, file)
        if m === nothing
            continue
        end
        L_ = parse(Int64, m.captures[1])
        dist_ = String(m.captures[2])
        param_ = parse(Float64, m.captures[4])

        if L == L_ && dist == dist_ && param == param_
            return joinpath(srcdir, file)
        end
    end
    return nothing
end

L = 50
dist = "Inverse"
param = 7

param_names = Dict("Weibull" => "k", "Inverse" => "a")
param_name = param_names[dist]

outdir = "C:/Users/javgua/Desktop/TFM/outputs/isosurfaces/"
pattern = r"^isosurfaces_L(\d+)_(\w+)\((\w)=([\d.]+)\).out$"
file = find_file(outdir, L, dist, param)
surface = load_surface(file, L, 104)

cmap = cgrad([:white, :black])

heatmap(
    surface,
    c=cmap,
    title="$dist($param_name=$param)",
    colorbar=true,
    aspect_ratio=1,
    size=(500, 400),
    clims=(0, maximum(surface)),
    # grid=true,
    xlims=(1, L),
    ylims=(1, L),
)

x_ticks = [1, L / 2, L]
y_ticks = [1, L / 2, L]
x_ticklabels = [L"1", L"L/2", L"L"]
y_ticklabels = [L"1", L"L/2", L"L"]

xticks!(x_ticks, x_ticklabels)
yticks!(y_ticks, y_ticklabels)
