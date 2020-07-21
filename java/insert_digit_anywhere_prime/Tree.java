package insert_digit_anywhere_prime;

import java.util.ArrayList;
import java.util.Arrays;

class Tree
{
    private long value;
    private ArrayList<Tree> children;

    public Tree(long tree_value, ArrayList<Long> tree_children)
    {
        value = tree_value;
        children = new ArrayList<Tree>();
        for (long child : tree_children) {
            children.add(new Tree(child, new ArrayList<Long>()));
        }
    }

    public void step()
    {
        if (children.isEmpty()) {
            ArrayList<Long> xs = next(value);
            for (long val : xs) {
                children.add(new Tree(val, new ArrayList<Long>()));
            }
            return;
        }
        for (Tree child : children) {
            child.step();
        }
    }

    public ArrayList<Long> longestPath()
    {
        ArrayList<Long> retval = new ArrayList<Long>();
        if (children.isEmpty()) {
            retval.add(value);
            return retval;
        }
        int max_length = 0;
        for (Tree child : children) {
            ArrayList<Long> temp = child.longestPath();
            if (temp.size() > max_length) {
                max_length = temp.size();
                retval = temp;
            }
        }
        retval.add(0, value);
        return retval;
    }

    /* modify this function to change functionality */
    public ArrayList<Long> next(long val)
    {
        /* Super messy but does work
         * TODO cleaning/comment
         */
        ArrayList<Long> new_xs = new ArrayList<Long>();
        String[] s =  Long.toString(val).split("");
        for(int i = 0; i <= s.length; i++){
            String[] temp = new String[s.length + 1];
            int j;
            for(j = 0; j < i; j++){
                temp[j] = s[j];
            }
            for(int k = j + 1; k < temp.length; k++){
                temp[k] = s[k - 1];
            }
            
            for(int num = 0; num <= 9; num++){
                temp[j] = String.valueOf(num);
                String str = Arrays.toString(temp)
                                   .replaceAll("[^\\d.]", "")
                                   .replaceAll("^0+(?!$)", "");
                Long test = Long.parseLong(str);
                if(Util.isPrime(test) && test != val){
                    new_xs.add(test);
                }
            }
        }
        return new_xs; 
    }
}
