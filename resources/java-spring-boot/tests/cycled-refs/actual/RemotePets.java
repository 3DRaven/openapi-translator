import java.util.List;

public class RemotePets {
    private List<RemotePetsitems> items;
    public RemotePets() {}
    public RemotePets(List<RemotePetsitems> items) {
        this.items = items;
    }
    public List<RemotePetsitems> getRemotePets() {
        return items;
    }
    public void setRemotePets(List<RemotePetsitems> items) {
        this.items = items;
    }
}