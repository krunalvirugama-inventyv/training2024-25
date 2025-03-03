Main URL = https://rustpro.onrender.com/

## User
```
/user/login
/user/register
```

## resources
```
/resources/create_resource
/resources/delete_resource
/resources/update_resource
/resources/get_resources
````

## disaster
```
/disaster/add_disaster_record
/disaster/add_do/{dr_id}
/disaster/add_dont/{dr_id}
/disaster/get_disaster_record/{dr_id}
/disaster/update_do/{dr_id}/{gi_id}
/disaster/update_dont/{dr_id}/{gi_id}
/disaster/get_all_disaster_record/{dr_id}
```

## shelter
```
/shelters/get_shelters
/shelters/create_shelter
/shelters/delete_shelter
/shelters/update_shelter
```

----------------------------------------------

## Response Format :
```
{
    status: true/false,
    message: "Message Here",
    data : []
}
```

## if success : 
```
{
    status: true,
    message: "Message Here",
    data : [{},{}]
}
```

## if error : 
```
{
    status: false,
    message: "Message Here",
    data : []
}
```
