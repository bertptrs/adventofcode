void doRotate(List<String> list, String arg) {
	def amount = Integer.valueOf(arg.substring(1))

	Collections.rotate(list, amount)
}

void exchangePos(List<String> list, String arg) {
	def parts = arg.substring(1).split("/")

	list.swap(Integer.valueOf(parts[0]), Integer.valueOf(parts[1]))
}

void exchangeProg(List<String> list, String arg) {
	def p1 = list.indexOf(arg.substring(1, 2));
	def p2 = list.indexOf(arg.substring(3, 4));

	list.swap(p1, p2);
}

def input = System.in.newReader().readLine()

def moves = input.split(",")

def state = new LinkedList(Arrays.asList("abcdefghijklmnop".split("")))

String dance(List<String> state, String[] moves) {
	for (move in moves) {
		switch (move.charAt(0)) {
			case 's':
				doRotate(state, move)
					break;

			case 'x':
				exchangePos(state, move)
					break;

			case 'p':
				exchangeProg(state, move)
					break;

			default:
				println("Unknown type!")
		}
	}

	return state.join("")
}

String danceOften(String[] moves, Integer amount) {
	def state = new LinkedList(Arrays.asList("abcdefghijklmnop".split("")))

	def seen = new HashSet<String>()
	def order = new ArrayList<String>()

	def summary = state.join("");

	for (i in 0..<amount) {
		if (seen.contains(summary)) {
			return order.get(amount % i)
		}

		seen.add(summary)
		order.add(summary)
		summary = dance(state, moves)
	}

	return summary;
}


println dance(state, moves)
println danceOften(moves, 1000000000)
