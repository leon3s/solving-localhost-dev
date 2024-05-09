import axios from "axios"

const API_URL = process.env.API_URL

console.log("using API_URL: ", API_URL)

export default axios.create({
  baseURL: API_URL,
  withCredentials: true,
})
