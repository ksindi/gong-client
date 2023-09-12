# \DefaultApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_generic_crm_integration_deprecated**](DefaultApi.md#delete_generic_crm_integration_deprecated) | **DELETE** /v2/crm/integration/delete | 
[**get_crm_objects_deprecated**](DefaultApi.md#get_crm_objects_deprecated) | **GET** /v2/crm/object/list | 
[**list_crm_schema_fields_deprecated**](DefaultApi.md#list_crm_schema_fields_deprecated) | **GET** /v2/crm/object/schema/list | 
[**list_generic_crm_integration_deprecated**](DefaultApi.md#list_generic_crm_integration_deprecated) | **GET** /v2/crm/integration/list | 
[**map_crm_users**](DefaultApi.md#map_crm_users) | **POST** /v2/crm/map/users | 
[**register_generic_crm_integration_deprecated**](DefaultApi.md#register_generic_crm_integration_deprecated) | **PUT** /v2/crm/integration/new | 
[**upload_crm_data_deprecated**](DefaultApi.md#upload_crm_data_deprecated) | **POST** /v2/crm/object/entities | 
[**upload_crm_schema_field_deprecated**](DefaultApi.md#upload_crm_schema_field_deprecated) | **POST** /v2/crm/object/schema | 
[**upload_stages**](DefaultApi.md#upload_stages) | **POST** /v2/crm/stages | 



## delete_generic_crm_integration_deprecated

> crate::models::BaseResponse delete_generic_crm_integration_deprecated(integration_id, client_request_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **i64** |  | [required] |
**client_request_id** | **String** |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_crm_objects_deprecated

> crate::models::BaseResponse get_crm_objects_deprecated(integration_id, object_type, objects_crm_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **i64** |  | [required] |
**object_type** | **String** |  | [required] |
**objects_crm_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_crm_schema_fields_deprecated

> crate::models::BaseResponse list_crm_schema_fields_deprecated(integration_id, object_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **i64** |  | [required] |
**object_type** | **String** |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_generic_crm_integration_deprecated

> crate::models::BaseResponse list_generic_crm_integration_deprecated()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## map_crm_users

> crate::models::BaseResponse map_crm_users(client_request_id, integration_id, data_file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_request_id** | **String** |  | [required] |
**integration_id** | **i64** |  | [required] |
**data_file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_generic_crm_integration_deprecated

> crate::models::BaseResponse register_generic_crm_integration_deprecated(generic_crm_registration_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generic_crm_registration_request** | [**GenericCrmRegistrationRequest**](GenericCrmRegistrationRequest.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_crm_data_deprecated

> crate::models::BaseResponse upload_crm_data_deprecated(client_request_id, integration_id, object_type, data_file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_request_id** | **String** |  | [required] |
**integration_id** | **i64** |  | [required] |
**object_type** | **String** |  | [required] |
**data_file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_crm_schema_field_deprecated

> crate::models::BaseResponse upload_crm_schema_field_deprecated(integration_id, object_type, generic_schema_field_request_deprecated)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **i64** |  | [required] |
**object_type** | **String** |  | [required] |
**generic_schema_field_request_deprecated** | [**Vec<crate::models::GenericSchemaFieldRequestDeprecated>**](GenericSchemaFieldRequestDeprecated.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_stages

> crate::models::BaseResponse upload_stages(integration_id, generic_deal_stage_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **i64** |  | [required] |
**generic_deal_stage_request** | [**Vec<crate::models::GenericDealStageRequest>**](GenericDealStageRequest.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

