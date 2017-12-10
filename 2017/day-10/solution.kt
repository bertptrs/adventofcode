fun valArray(): IntArray
{
    val arr = IntArray(256)
    for (i in arr.indices) {
        arr[i] = i
    }

    return arr
}

fun hash(input: IntArray, values: IntArray, index: Int, skip: Int): Pair<Int, Int> {
    var curIndex = index
    var curSkip = skip
    for (range in input) {
        val halfRange = range / 2

        for (i in 0..(halfRange - 1)) {
            val i1 = (curIndex + i) % values.size
            val i2 = (curIndex + range - i - 1) % values.size

            values[i1] = values[i2].also { values[i2] = values[i1] }
        }

        curIndex = (curIndex + range + curSkip) % values.size
        curSkip++
    }

    return Pair(curIndex, curSkip)
}

fun solve(input: String)
{
    // Part 1
    val part1input = input.split(",").map { it.toInt() }.toIntArray()
    val values = valArray()
    hash(part1input, values, 0, 0)
    println(values[0] * values[1])

    // Part 2
    val part2input = ArrayList(List(input.length) { input[it].toInt()})
    part2input.addAll(arrayOf(17, 31, 73, 47, 23))

    val values2 = valArray()
    var skip = 0
    var index = 0

    // Repeat hash 64 times
    for (i in 1..64) {
        val (newIndex, newSkip) = hash(part2input.toIntArray(), values2, index, skip)
        index = newIndex
        skip = newSkip
    }

    for (i in 0..15) {
        val startIndex = 16 * i
        val endIndex = startIndex + 15

        val currentNum = values2.slice(startIndex..endIndex).reduce{a, b -> a.xor(b)}
        print("%02x".format(currentNum))
    }
    println()

}

fun main(args: Array<String>)
{
    solve(readLine()!!)
}