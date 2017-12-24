import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class solution {

    private final Map<Integer, List<Connection>> connections;

    private solution(Map<Integer, List<Connection>> connections) {
        this.connections = connections;
    }

    private int part1() {
        return getMaxScore(0, new HashSet<>());
    }

    private int getMaxScore(int startingPoint, HashSet<Connection> used) {
        int best = 0;

        for (Connection c : connections.get(startingPoint)) {
            if (used.contains(c)) {
                continue;
            }

            used.add(c);
            best = Math.max(best, getMaxScore(c.getOther(startingPoint), used) + c.score());
            used.remove(c);
        }

        return best;
    }

    private int part2() {
        return getMaxLongest(0, new HashSet<>()).getKey();
    }

    private Map.Entry<Integer, Integer> getMaxLongest(int startingPoint, HashSet<Connection> used) {
        int strongest = 0;
        int longest = 0;

        for (Connection c : connections.get(startingPoint)) {
            if (used.contains(c)) {
                continue;
            }

            used.add(c);
            Map.Entry<Integer, Integer> res = getMaxLongest(c.getOther(startingPoint), used);
            used.remove(c);

            if (longest < res.getValue() + 1) {
                longest = res.getValue() + 1;
                strongest = res.getKey() + c.score();
            } else if (longest == res.getValue() + 1) {
                strongest = Math.max(res.getKey() + c.score(), strongest);
            }
        }

        return new AbstractMap.SimpleEntry<>(strongest, longest);
    }

    public static void main(String[] args) throws IOException {
        solution s = new solution(readInput());

        System.out.println(s.part1());
        System.out.println(s.part2());
    }

    private static Map<Integer, List<Connection>> readInput() throws IOException {
        final Map<Integer, List<Connection>> connections = new HashMap<>();

        try (BufferedReader in = new BufferedReader(new InputStreamReader(System.in))) {
            in.lines()
                    .map(s -> s.trim().split("/"))
                    .map(Connection::new)
                    .forEach(c -> {
                        connections.computeIfAbsent(c.getValue(), a -> new ArrayList<>())
                                .add(c);
                        connections.computeIfAbsent(c.getKey(), a -> new ArrayList<>())
                                .add(c);
                    });
        }

        return Collections.unmodifiableMap(connections);
    }

    private static class Connection extends AbstractMap.SimpleImmutableEntry<Integer, Integer> {

        Connection(String[] parts) {
            this(Integer.valueOf(parts[0]), Integer.valueOf(parts[1]));
        }

        Connection(Integer integer, Integer integer2) {
            super(Math.min(integer, integer2), Math.max(integer, integer2));
        }

        int getOther(int first) {
            return getKey() == first ? getValue() : getKey();
        }

        int score() {
            return getKey() + getValue();
        }
    }
}
