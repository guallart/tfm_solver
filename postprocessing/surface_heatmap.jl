using Plots
using Plots.PlotMeasures
using LaTeXStrings

function load_surfaces(file, L)
    surfaces = zeros(Float64, L, L)
    n_surfaces = 0

    open(file) do f
        _ = readline(f) # Skip the first line
        for line in eachline(f)
            coords = split(line, " ")
            if length(coords) % 2 != 0
                println("Invalid line: $line")
                continue
            end
            cols = 2 .+ parse.(Int64, coords[2:2:end])
            rows = 1 .+ parse.(Int64, coords[1:2:end])
            n_surfaces += 1
            for i in eachindex(cols)
                surfaces[rows[i], cols[i]] += 1
            end
        end
    end
    return surfaces ./ n_surfaces
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

L = 100
dist = "Inverse"
param = 15

if dist == "Weibull"
    param_name = "k"
elseif dist == "Inverse"
    param_name = "a"
else
    error("Unknown distribution: $dist")
end

outdir = "C:/Users/javgua/Desktop/TFM/outputs/isosurfaces/"
pattern = r"^isosurfaces_L(\d+)_(\w+)\((\w)=([\d.]+)\).out$"
file = find_file(outdir, L, dist, param)
surfaces = load_surfaces(file, L)


x_ticks = [1, L / 2, L]
y_ticks = [1, L / 2, L]
x_ticklabels = [L"1", L"L/2", L"L"]
y_ticklabels = [L"1", L"L/2", L"L"]

cmap = cgrad([:white, :black])

heatmap(
    surfaces,
    c=cmap,
    title="$dist($param_name=$param)",
    colorbar=true,
    colorbar_title="\nProbabilidad",
    aspect_ratio=1,
    size=(500, 400),
    clims=(0, maximum(surfaces)),
    grid=false,
    xlims=(1, L),
    ylims=(1, L),
)

xticks!(x_ticks, x_ticklabels)
yticks!(y_ticks, y_ticklabels)
