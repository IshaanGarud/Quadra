# Pnemonics
    - Source = Original / Target image
    - Chunk = A buffer of size m*n (or n*n if it's a square. Preferrable)
    - Average of a chunk (avg) = [Sigma(source_pixel_color)] / (no. of pixels in the chunk)
    - Mean Square Error (mse) = 
        -- D_i = (Ri - R_avg)^2 + (Gi - G_avg)^2 + (Bi - B_avg)^2
        -- MSE = [ Sigma(Di) ] / 3n
    - Threshold = The minimum threshold for error required, after which the chunk is split, and each chunk is colored.


# Essentials
## Which Approach?
        -- Iterative:- Scan level n, generate chunk for level n. Move to level n+1. One level at a time.
        -- Recursive:- Split a chunk until it cannot be split further. Once it's done move to next chunk.

## How to split to chunk?
        1. Take a chunk
        2. Determine the average color of the original chunk.
        3. Determine the MSE of the chunk wrt corresponding chunk from original image.
        4. Evaluate and split + recolor , or skip.
            --- Color:- chunk_color = average of color of all pixels in the chunk
        5. Go to next chunk
        
        ! load -> check -> split or skip
        ! First color a chunk, then run the error function. After that do split or skip. Repeat.

# Implementation
    - Define a struct Chunk.
    - Don't copy the pixel data inside each chunk that'll make copying harder.
    - Make 2 buffers. Immutable `original_image_buffer` for calculating the average color of that original_chunk.
        Mutable `output_buffer`.
    
    - Average color Rgb<u8> -> Average of the chunk of the ORIGINAL IMAGE.
    - MSE: <scalar> -> Sum of square of the difference between chunk_pixel_color (source wala) and the average.
    Summary ->
        -- Avg => Only from the original image
        -- MSE => Only from the output_image_pixel and avg.

* Chunks to Image