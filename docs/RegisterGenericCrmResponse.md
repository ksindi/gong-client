# RegisterGenericCrmResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**integration_id** | Option<**i64**> | <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style>Your integration ID to be used in requests to the API. This ensures you are accessing the correct integration.<br><b>Note:</b> Parse the integrationId as Long or BigInt. Parsing the integrationId as an Integer will truncate the integrationId to trailing zeros and will return an incorrect ID. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


