import org.springframework.transaction.annotation.Transactional;
import java.util.List;

public class PagingSimplifiedTrackObjectAnySchemaAllOf {

    private  List<CustomCode> items = new List<>();
    @Transactional
    private  String security;

}
