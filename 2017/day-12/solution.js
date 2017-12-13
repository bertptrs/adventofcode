const stdin = process.openStdin();
let content = '';

Set.prototype.union = function(setB) {
	var union = new Set(this);
	for (var elem of setB) {
		union.add(elem);
	}
	return union;
}

function parseConnections(data) {
	let connections = {};

	data.split("\n").forEach(line => {
		if (!line) {
			return;
		}

		let from, to;
		[from, to] = line.split(" <-> ");
		connections[from] = to.split(", ");
	});

	return connections;
}

function getConnected(from, connections) {
	let todo = [from];
	let connected = new Set(todo);

	for (let cur = 0; cur < todo.length; ++cur) {
		if (!connections[todo[cur]]) {
			continue;
		}

		connections[todo[cur]].forEach(n => {
			if (!connected.has(n)) {
				connected.add(n);
				todo.push(n);
			}
		});
	}

	return connected;
}

stdin.addListener('data', d => {
	content += d.toString();
});

stdin.addListener('end', () => {
	let connections = parseConnections(content);

	let connected = getConnected("0", connections);

	console.log(connected.size);

	let groups = 1;

	for (let n in connections) {
		if (connected.has(n)) {
			continue;
		}

		connected = connected.union(getConnected(n, connections));
		++groups;
	}
	console.log(groups);
});
