import java.util.ArrayList;

/* Basic tree stuff. I wouldn't recommend tweaking this unless you have some
 * ideas for optimizations. Or if my code is bad; it's been a long while since I
 * wrote Java. However, please modify `next` below to change functionality as
 * described in the Overleaf
 */
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

    /* TODO please tweak this function according to what you want the code to do */
    public ArrayList<Long> next(long val)
    {
        ArrayList<Long> new_xs = new ArrayList<Long>();
        return new_xs;
    }
}
