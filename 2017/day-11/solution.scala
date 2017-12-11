import scala.io.StdIn

object solution {
  def main(args: Array[String]) {
    val input = StdIn.readLine()
    val dirs = input.trim.split(",")

    follow(dirs)
  }

  def follow(dirs: Array[String]): Unit = {
    var x = 0
    var y = 0

    var maxDist = 0

    for (dir <- dirs) {
      dir match {
        case "s" => y -= 1
        case "n" => y += 1
        case "nw" => x -= 1
        case "se" => x += 1
        case "ne" => y += 1; x += 1
        case "sw" => y -= 1; x -= 1
      }

      maxDist = Math.max(maxDist, dist(x, y))
    }

    println(dist(x, y))
    println(maxDist)
  }

  def dist(x: Int, y: Int): Int = {
    return Math.max(x, y);
  }
}
