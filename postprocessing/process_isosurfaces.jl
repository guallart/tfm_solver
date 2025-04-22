using Printf
using Statistics
using Plots
using Plots.PlotMeasures
using LaTeXStrings

function compute_metrics(surfaces_file)
    ℓ::Vector{Int64} = []
    h::Vector{Int64} = []
    new_ℓ = 0
    new_h = 0
    try
        open(surfaces_file) do file
            for line in eachline(file)
                try
                    coords = split(line, " ")
                    cols = coords[2:2:end]
                    new_ℓ = length(cols)
                    new_h = abs(parse(Int32, cols[1]) - parse(Int32, cols[end]))
                    push!(ℓ, new_ℓ)
                    push!(h, new_h)
                catch e
                    continue
                end
            end
        end
        return (mean(ℓ), mean(h))
    catch e
        println(e)
        return (NaN64, NaN64)
    end
end

function load_files(srcdir)
    files = readdir(srcdir)
    ℓs = Dict()
    hs = Dict()

    for file in files
        m = match(pattern, file)
        if m === nothing
            continue
        end
        L = parse(Int64, m.captures[1])
        dist = String(m.captures[2])
        param = parse(Float64, m.captures[4])

        surfaces_file = joinpath(srcdir, file)
        ℓ, h = compute_metrics(surfaces_file)

        ℓs[dist, L, param] = ℓ
        hs[dist, L, param] = h
        @printf("L=%d   dist=%s   param=%.3f   ℓ=%.3f   h=%.3f\n", L, dist, param, ℓ, h)
    end

    return ℓs, hs
end

get_keys(index, dist) = [key[index] for key in keys(ℓs) if key[1] == dist] |> Set |> collect |> sort

Lx_inv(a) = (a * 0.5 / log(2) - 1)^(4 / 3)
Lx_wei(k) = ((1 - 0.5^(2^(-k))) / (-0.5 + 0.5^(2^(-k))))^(4 / 3)
dopt = 1.22

Lx = Dict(
    "Inverse" => Lx_inv,
    "Weibull" => Lx_wei
)

pattern = r"^isosurfaces_L(\d+)_(\w+)\((\w)=([\d.]+)\).out$"
# srcdir = "C:/Users/javgua/Desktop/TFM/outputs/isosurfaces/"
srcdir = "C:/Users/javgua/Desktop/TFM/solver/runner/isosurfaces/"

ℓs, hs = load_files(srcdir)
dists = Set([key[1] for key in keys(hs)])

println("\n\n\nPlotting\n")

markers = [
    :circle
    :cross
    :diamond
    :dtriangle
    :heptagon
    :ltriangle
    :pentagon
    :rect
    :rtriangle
    :star4
    :star5
    :utriangle
    :x
    :xcross
]

p1 = plot()
marker_index = 1
for dist in dists
    Ls = get_keys(2, dist)
    params = get_keys(3, dist)
    for p in params
        try
            L_c = Lx[dist](p)
            x = [Li / L_c for Li in Ls if (dist, Li, p) in keys(ℓs)]
            y = [ℓs[(dist, Li, p)] for Li in Ls if (dist, Li, p) in keys(ℓs)] ./ L_c^dopt
            plot!(x, y, label="$dist($p)", legend=:bottomright, marker=markers[marker_index])
            global marker_index
            marker_index = marker_index % length(markers) + 1
        catch e
            println("Failed $dist(p=$p)   error: $e")
        end
    end
end

slope = dopt
xd = 10 .^ [-1.5, -0.1]
yd = @. 10^(slope * log10(xd) + 0.2)
plot!(xd, yd, linestyle=:dash, color=:black, label="", legend=:bottomright)
annotate!(5e-2, mean(yd) - 0.3, text("slope $(slope)", 10))

slope = 1.0
xd = 10 .^ [0.1, 1.5]
yd = @. 10^(slope * log10(xd) + 0.2)
plot!(xd, yd, linestyle=:dash, color=:black, label="", legend=:bottomright)
annotate!(4.0, mean(yd) - 0.3, text("slope $(slope)", 10))

xaxis!(:log10)
yaxis!(:log10)
xlabel!(L"L / L_{\times}")
ylabel!(L"⟨ℓ⟩ / L_{\times}^{d_{opt}}")


p2 = plot()
marker_index = 1
for dist in dists
    Ls = get_keys(2, dist)
    params = get_keys(3, dist)
    for p in params
        try
            L_c = Lx[dist](p)
            x = [Li / L_c for Li in Ls if (dist, Li, p) in keys(ℓs)]
            y = [hs[(dist, Li, p)] for Li in Ls if (dist, Li, p) in keys(ℓs)] ./ L_c
            plot!(x, y, label="$dist($p)", legend=:bottomright, marker=markers[marker_index])
            global marker_index
            marker_index = marker_index % length(markers) + 1
        catch e
            println("Failed $dist(p=$p)   error: $e")
        end
    end
end

slope = 1.0
xd = [10^-1.5, 10^-0.5]
yd = @. 10^(slope * log10(xd) - 0.9)
plot!(xd, yd, linestyle=:dash, color=:black, label="", legend=:bottomright)
annotate!(3e-1, 1e-2, text("slope $(slope)", 10))

slope = 0.1
xd = [7, 70]
yd = @. 10^(slope * log10(xd) - 0.5)
plot!(xd, yd, linestyle=:dash, color=:black, label="")
annotate!(30, 0.3, text("slope $(slope)", 10))

xaxis!(:log10)
yaxis!(:log10)
xlabel!(L"L / L_{\times}")
ylabel!(L"⟨h⟩ / L_{\times}")

plot(p1, p2, layout=(1, 2), size=(1000, 400), margin=[5mm 5mm])
# savefig("resultados.pdf")