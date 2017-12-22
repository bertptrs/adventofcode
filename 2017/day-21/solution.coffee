#!/usr/bin/env coffee
String.prototype.count = (pattern) ->
	(this.match(pattern) || []).length

stdin = process.openStdin()

stdin.setEncoding 'utf8'

data = ''

stdin.on 'data', (input) ->
	data += input

stdin.on 'end', () ->
	mappings = {}

	rotate = (line) ->
		rows = line.split '/'

		newrows = []

		len = rows.length

		for i in [0...len]
			newrows[i] = ''
			for j in [0...len]
				newrows[i] += rows[rows.length - 1 - j].substr i, 1

		newrows.join '/'

	flip = (line) ->
		(row.split('').reverse().join '' for row in line.split '/').join '/'

	view = (state) ->
		console.log state.split('/').join "\n"

	for line in data.trim().split('\n')
		do (line) ->
		[source, enhanced] = line.split ' => '

		for i in [1..4]
			mappings[source] = enhanced
			mappings[flip source] = enhanced
			source = rotate source

	state = '.#./..#/###'
	view state

	for _ in [1..18]
		newrows = []
		oldrows = state.split '/'

		if oldrows.length % 2 == 0
			for i in [0...(oldrows.length / 2)]
				currows = ['', '', '']

				for j in [0...(oldrows.length / 2)]
					curblock = (oldrows[2 * i].substr 2 * j, 2) + '/' + (oldrows[2 * i + 1].substr 2 * j, 2)
					blockrows = mappings[curblock].split '/'

					for r in [0..2]
						currows[r] += blockrows[r]

				for row in currows
					newrows.push row
		else
			for i in [0...(oldrows.length / 3)]
				currows = ['', '', '', '']

				for j in [0...(oldrows.length / 3)]
					curblock = (oldrows[3 * i].substr 3 * j, 3) + '/' + (oldrows[3 * i + 1].substr 3 * j, 3) + '/' + (oldrows[3 * i + 2].substr 3 * j, 3)
					blockrows = mappings[curblock].split '/'

					for r in [0..3]
						currows[r] += blockrows[r]

				for row in currows
					newrows.push row

		state = newrows.join '/'
		console.log _, state.count(/#/g)
