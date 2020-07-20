package append_primes;

import java.util.HashMap;
import java.util.Iterator;
import java.util.stream.IntStream;
import java.util.ArrayList;

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
        // Add 2 to the HashMap in advance in order to quickly exclude even number later
        primes.put(2l, true);
        if (primes.containsKey(val)) {
            return primes.get(val);
        }
        if (val < 2) {
            primes.put(val, false);
            return false;
        }
        /* I tried to make this process more readable but I'm not sure about the efficiency 
         * TODO check the efficiency of this new method
         */
        boolean temp = (val % 2) != 0 
                        && 
                        IntStream.rangeClosed(3, (int) Math.sqrt(val))
                        .filter(n -> n % 2 != 0)
                        .noneMatch(n -> (val % n == 0));
        primes.put(val, temp);
        return temp;
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

    // Did some cleaning to this util method
    public static void printList(ArrayList<Long> ls)
    {
        Iterator<Long> it = ls.iterator();
        System.out.printf("[%s",it.next());
        do{
            System.out.printf(", %s", it.next());
        }while(it.hasNext());
        System.out.print("]\n");
    }
}
