using DelimitedFiles
using Plots
using Printf

function first_neighbors(L)
    neighbours = [[] for i = 1:L, j = 1:L]
    offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    for i = 1:L
        for j = 1:L-2
            for (di, dj) in offsets
                ni, nj = i + di, j + dj
                if (1 <= ni <= L) && (1 <= nj <= L)
                    push!(neighbours[i, j], (ni, nj))
                end
            end
        end
    end

    return neighbours
end

function find_zero_curve(nodes, neighs)
    rows, cols = size(nodes)
    curve = []
    for i = 1:rows
        for j = 1:cols
            if nodes[i, j] <= 0
                continue
            end

            for k in neighs[i, j]
                if nodes[k...] < 0
                    push!(curve, (i, j))
                    break
                end
            end
        end
    end
    return curve
end

function get_or_compute_neighs!(dict::Dict, L)
    if haskey(dict, L)
        return dict[L]
    else
        dict[L] = first_neighbors(L)
        return dict[L]
    end
end

pattern = r"^L(\d+)_ExpDist\(a=([\d.]+)\)_(\d+)\.x$"
srcdir = "C:/Users/javgua/Desktop/TFM/outputs/rx/"
dstdir = "C:/Users/javgua/Desktop/TFM/outputs/isosurfaces_temp/"
files = readdir(srcdir)

neighs_dict = Dict()

for file in files
    m = match(pattern, file)

    if m !== nothing
        try
            L = parse(Int64, m.captures[1])
            param = parse(Float64, m.captures[2])
            case = parse(Int64, m.captures[3])

            x = readdlm(joinpath(srcdir, file), skipstart=6)
            nodes = reshape(x, (L, L - 2))
            neighs = get_or_compute_neighs!(neighs_dict, L)
            path = find_zero_curve(nodes, neighs)
            line = result = join(["$a $b" for (a, b) in path], " ")
            dstname = @sprintf("isosurfaces_L%d_ExpDist(a=%.3f).out", L, param)
            dstfile = joinpath(dstdir, dstname)
            open(dstfile, "a") do outfile
                println(outfile, line)
            end
        catch
            println("Failed processing file $file")
        end
    end
end
