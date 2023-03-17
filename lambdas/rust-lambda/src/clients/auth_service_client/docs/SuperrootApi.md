# \SuperrootApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data**](SuperrootApi.md#get_data) | **GET** / | 
[**post_data**](SuperrootApi.md#post_data) | **POST** / | 



## get_data

> Vec<crate::models::GetResponse> get_data()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GetResponse>**](GetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_data

> crate::models::CommandResponse post_data(command_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**command_request** | [**CommandRequest**](CommandRequest.md) |  | [required] |

### Return type

[**crate::models::CommandResponse**](CommandResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

