type num = u64

def next_secret_number (n: num) = n
    |> (\x -> x ^ (x << 6))
    |> (& 0x00FF_FFFF)
    |> (\x -> x ^ (x >> 5))
    |> (\x -> x ^ (x << 11))
    |> (& 0x00FF_FFFF)

def main (nums: []num) = nums
    |> map (iterate 2000 next_secret_number)
    |> reduce (+) 0 -- sum
