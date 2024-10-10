import java.util.List;

public class LocalPets {
    private List<LocalPetsitems> items;
    public LocalPets() {}
    public LocalPets(List<LocalPetsitems> items) {
        this.items = items;
    }
    public List<LocalPetsitems> getLocalPets() {
        return items;
    }
    public void setLocalPets(List<LocalPetsitems> items) {
        this.items = items;
    }
}