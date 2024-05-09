import axios from "axios"

export default axios.create({
  baseURL: process.env.API_URL || "http://dev.next-hat.wtf:8181",
  withCredentials: true,
})
