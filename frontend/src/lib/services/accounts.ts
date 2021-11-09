import axios, {AxiosInstance} from 'axios';

const axiosInstance = axios.create({
    baseURL: import.meta.env.VITE_API_HOST as string
})

const config = {
    headers: {
        authorization: ""
    }
}

// function to execute the http get request
//const get = (url, request) => apiRequest("get", url);

// function to execute the http delete request
//const deleteRequest = (url, request) =>  apiRequest("delete", url, request);

//// function to execute the http post request
//const post = (url, request) => apiRequest("post", url, request);

//// function to execute the http put request
//const put = (url, request) => apiRequest("put", url, request);

// expose your method to other services or actions
const API = {
    get,
    //delete: deleteRequest,
    //post,
    //put,
    //patch
};
export default API;
