helpFunction()
{
    echo "This script requires exactly two arguments"
    echo "USAGE: $0 <DB_USERNAME> <DB_PASSWORD>"
    echo "DB_USERNAME and DB_PASSWORD must be valid on the database at localhost:5984"
    exit 1
}

if [ -z "$1" ]
    then
        helpFunction
fi

if [ -z "$2" ]
    then
        helpFunction
fi

curl -X DELETE http://$1:$2@localhost:5984/product

curl -X PUT http://$1:$2@localhost:5984/product

curl -X POST http://$1:$2@localhost:5984/product/_bulk_docs \
    -H "Content-type: application/json" \
    -d '{
        "docs": [
            {
                "_id": "123",
                "value": 1235,
                "currency_code": "USD"
            }
        ]}'