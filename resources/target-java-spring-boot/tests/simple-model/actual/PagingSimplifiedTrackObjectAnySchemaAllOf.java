import org.springframework.transaction.annotation.Transactional;

public class PagingSimplifiedTrackObjectAnySchemaAllOf {

    @Transactional
    private  String another_field;
    private  String some_field;
    private  List<CustomCode> items = new List<>();
    @Transactional
    private  String security;

}
