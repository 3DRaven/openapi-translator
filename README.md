# openapi-translator
Translator from OpenAPI v3 to some code



1. Пройти через схему, определить имя
2. Пройти в массив, он пишется в ту же модель

type: array
items:
    type: string

val array: List<         String            >

Model:
  type: array
  items:
    type: array
    items:
      type: integer

val array: List<        List<String>       >


{
  "p":[{id}]
}

Model:
  type: object
  properties:
    p:
      type: array
      items:
        type: object
        properties:
          id:
            type: integer

#С разбиением на модели, методы возвращают [(code,model),(code,model)]
Model:
  type: object_start [{Model,1}] >w
  properties: 
    ppp:
      type: string [{Model,1},{ppp,2}] >>w
    prop:         
      type: object_start [{Model,1},{prop,2}] >w
      properties:
          arr:      
            type: array_start [{Model,1},{prop,2},{arr,3}] >>w
            items: string [{Model,1},{prop,2},{arr,3}] >>w
            additionalProperties::
              type: object_start [{Model,1},{prop,2},{arr,3}] >w 
              properties:
                id:
                  type: integer [{Model,1},{prop,2},{arr,3},{id,4}] >>w
              type: object_end [{Model,1},{prop,2},{arr,3}] >w>w 
            type: array_end [{Model,1},{prop,2},{arr,3}] >>w            
      type: object_end [{Model,1},{prop,2}] >w>w
  type: object_end [{Model,1}] >w>-

#без разбиения на модели
Model:
  type: object [{Model,1}] >1
  properties: 
    ppp:
      type: string [{Model,1},{ppp,2}] >1
    prop:         
      type: object [{Model,1},{prop,2}] >1
      properties:
          arr:      
            type: array [{Model,1},{prop,2},{arr,3}] >1
            items: string [{Model,1},{prop,2},{arr,3}] >1
            items:
              type: object [{Model,1},{prop,2},{arr,3}] >1 
              properties:
                id:
                  type: integer [{Model,1},{prop,2},{arr,3},{id,4}] >1


val array: List<         Model               >

type: array
items:
  $ref: '#/components/schemas/Pet'

val array: $ref

