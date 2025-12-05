#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

 #define MAX(a,b) \
    ({ __typeof__ (a) _a = (a); \
        __typeof__ (b) _b = (b); \
        _a > _b ? _a : _b; })

typedef struct {
    uint64_t start;
    uint64_t end;
} range_t;

typedef struct {
    size_t length;
    size_t capacity;
    range_t* buffer;
} ranges_t;

static int cmp_range(void const* va, void const* vb) {
    range_t const* a = va;
    range_t const* b = vb;

    if (a->start == b->start) {
        return (a->end > b->end) - (a->end < b->end);
    } else {
        return (a->start > b->start) - (a->start < b-> start);
    }
}

static void simplify_ranges(ranges_t* ranges) {
    // First, sort ranges ascending by start (then end)
    qsort(ranges->buffer, ranges->length, sizeof(range_t), cmp_range);

    const size_t length = ranges->length;
    range_t* write_ptr = &ranges->buffer[0];

    // Important, skip the first iteration, it can never be merged
    for (size_t i = 1; i < length; ++i) {
        if (ranges->buffer[i].start <= write_ptr->end) {
            // Merge the two ranges
            write_ptr->end = MAX(write_ptr->end, ranges->buffer[i].end);
            ranges->length--;
        } else {
            write_ptr++;
            if (write_ptr != &ranges->buffer[i]) {
                memcpy(write_ptr, &ranges->buffer[i], sizeof(range_t));
            }
        }
    }

    // for (size_t i = 0; i < ranges->length; ++i) {
    //     printf("%" PRIu64 "-%" PRIu64 "\n", ranges->buffer[i].start, ranges->buffer[i].end);
    // }

    // printf("Original length: %u, new length: %u\n", (unsigned int) length, (unsigned int) ranges->length);
}

bool is_fresh(const uint64_t ingredient, ranges_t const * const ranges) {
    size_t min = 0;
    size_t size = ranges->length;
    range_t const* buffer = ranges->buffer;

    while (size > 1) {
        size_t half = size / 2;
        size_t mid = min + half;

        if (buffer[mid].start <= ingredient) {
            min = mid;
        }

        size -= half;
    }

    return buffer[min].start <= ingredient && buffer[min].end >= ingredient;
}

int main(const int argc, char const** argv) {
    if (argc < 2) {
        printf("Usage: %s <input>\n", argv[0]);
        return 1;
    }

    ranges_t ranges = {
        .length = 0,
        .capacity = 2,
        .buffer = malloc(2 * sizeof(range_t)),
    };

    FILE* input = fopen(argv[1], "r");
    if (input == NULL) {
        perror("failed to open file");
        exit(1);
    }

    uint64_t ingredient;
    bool seen_first = false;

    while (!seen_first) {
        uint64_t start;
        uint64_t end;

        switch (fscanf(input, "%" PRIu64 "-%" PRIu64, &start, &end)) {
            case 1:
                seen_first = true;
                ingredient = start;
                break;

            case 2:
                if (ranges.capacity == ranges.length) {
                    size_t new_capacity = ranges.capacity * 2;
                    range_t* buffer = realloc(ranges.buffer, new_capacity * sizeof(range_t));
                    if (buffer == NULL) {
                        printf("Failed to resize ranges buffer\n");
                        return 1;
                    }

                    ranges.capacity = new_capacity;
                    ranges.buffer = buffer;
                }

                ranges.buffer[ranges.length].start = start;
                ranges.buffer[ranges.length].end = end;
                ranges.length++;
                break;
            default:
                printf("Unexpected end of file\n");
                return 1;
        }
    }

    simplify_ranges(&ranges);

    int total_fresh = 0;

    do {
        if (is_fresh(ingredient, &ranges)) {
            total_fresh++;
        }
    } while (fscanf(input, "%" PRIu64 "\n", &ingredient) == 1);

    printf("Part1: %u\n", total_fresh);

    uint64_t combined_fresh = 0;

    for (size_t i = 0; i < ranges.length; ++i) {
        combined_fresh += ranges.buffer[i].end - ranges.buffer[i].start + 1;
    }

    printf("Part2: %" PRIu64 "\n", combined_fresh);

    fclose(input);
    free(ranges.buffer);

    return 0;
}
