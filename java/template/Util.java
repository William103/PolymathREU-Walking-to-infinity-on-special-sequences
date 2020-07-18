import java.util.HashMap;
import java.util.ArrayList;

/* Just some utility functions like isPrime, isSquareFree, and printList, all of
 * which do exactly what they sound like. I wouldn't recommend tweaking this
 * unless you have some really good ideas for optimization
 */
class Util
{
    private static HashMap<Long, Boolean> primes;
    private static HashMap<Long, Boolean> squareFree;

    public static void init()
    {
        primes = new HashMap<Long, Boolean>();
        squareFree = new HashMap<Long, Boolean>();
    }

    public static boolean isPrime(long val)
    {
        if (primes.containsKey(val)) {
            return primes.get(val);
        }
        for (int i = 2; i * i <= val; i++) {
            if (val % i == 0) {
                primes.put(val, false);
                return false;
            }
        }
        primes.put(val, true);
        return true;
    }

    public static boolean isSquareFree(long val)
    {
        if (squareFree.containsKey(val)) {
            return squareFree.get(val);
        }
        for (int i = 2; i * i <= val; i++) {
            if (isPrime(i)) {
                if (val % (i * i) == 0) {
                    squareFree.put(val, false);
                    return false;
                }
            }
        }
        squareFree.put(val, true);
        return true;
    }

    public static void printList(ArrayList<Long> ls)
    {
        System.out.print("[");
        for (int i = 0; i < ls.size() - 1; i++) {
            System.out.print(ls.get(i));
            System.out.print(", ");
        }
        System.out.print(ls.get(ls.size() - 1));
        System.out.print("]\n");
    }
}
