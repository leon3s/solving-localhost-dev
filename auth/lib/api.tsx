import axios from "axios"

console.log(process.env)

export default axios.create({
  baseURL: process.env.API_URL || "http://dev.next-hat.wtf:8181",
  withCredentials: true,
})
