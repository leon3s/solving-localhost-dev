"use client"

import React from "react"
import axios from "@/lib/api"
import { redirect, useRouter, useSearchParams } from "next/navigation"

import PageTitle from "@/components/PageTitle"

export default function Signup() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [error, setError] = React.useState(null)
  const [isSubmitting, setIsSubmitting] = React.useState(false)

  const redirectUrl = searchParams.get("redirect") as string

  if (!redirectUrl) {
    redirect(process.env.DEFAULT_REDIRECT as string)
  }

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setIsSubmitting(true)
    const name = (e.currentTarget.elements[0] as HTMLInputElement).value
    const email = (e.currentTarget.elements[1] as HTMLInputElement).value
    const password = (e.currentTarget.elements[2] as HTMLInputElement).value
    const user = {
      name,
      email,
      password,
    }

    axios
      .post("/users", user)
      .then((res) => {
        return router.push(`/?redirect=${searchParams.get("redirect")}`)
      })
      .catch((err) => {
        const res = err.response
        setError(
          res?.data?.msg ||
            res?.data ||
            err.message ||
            "An unknown error occurred.",
        )
        setIsSubmitting(false)
      })
  }

  return (
    <main className="flex flex-col items-center justify-between">
      <PageTitle title="Sign up to NextHat" />
      <div className="w-[342px]">
        <form
          onSubmit={onSubmit}
          className="text-center flex flex-col bg-[#1b1b1d] p-[16px] rounded-[6px]"
        >
          <div className="relative z-50">
            <div className="mb-4">
              <label
                className="text-left block mb-2 text-[14px]"
                htmlFor="username"
              >
                Username
              </label>
              <input
                className="outline-0 w-full border border-[#30363d] rounded-[6px] bg-black py-[5px] px-[12px] focus:border-[var(--ifm-color-primary)] focus:outline-none focus:ring-1 focus:ring-[var(--ifm-color-primary)] text-[14px]"
                id="username"
                type="text"
              />
            </div>
            <div className="mb-4">
              <label
                className="text-left block mb-2 text-[14px]"
                htmlFor="email"
              >
                Email
              </label>
              <input
                className="outline-0 w-full border border-[#30363d] rounded-[6px] bg-black py-[5px] px-[12px] focus:border-[var(--ifm-color-primary)] focus:outline-none focus:ring-1 focus:ring-[var(--ifm-color-primary)] text-[14px]"
                id="email"
                type="email"
              />
            </div>
            <div className="mb-4">
              <label
                className="relative text-left block mb-2 text-[14px]"
                htmlFor="password"
              >
                Password
              </label>
              <input
                className="outline-0 w-full border border-[#30363d] rounded-[6px] bg-black py-[5px] px-[12px] focus:border-[var(--ifm-color-primary)] focus:outline-none focus:ring-1 focus:ring-[var(--ifm-color-primary)] text-[14px]"
                id="password"
                type="password"
              />
            </div>
            {error && (
              <div className="text-center mb-4">
                <p className="text-[12px] text-red-500">{error}</p>
              </div>
            )}
            {/* <p className="text-[12px] text-[#8e8e93] mb-4">
              By signing up, you agree to our{" "}
              <a
                className="text-[#007aff] hover:underline"
                href="https://nextjs.org/terms/"
              >
                Terms
              </a>{" "}
              and{" "}
              <a
                className="text-[#007aff] hover:underline"
                href="https://nextjs.org/privacy/"
              >
                Privacy Policy
              </a>
              .
            </p> */}
            <button
              className="transition ease-in-out w-full bg-orange-500 hover:bg-orange-700 text-white text-[14px] py-2 px-4 rounded-[6px] py-[5px] px-[16px] font-medium"
              type="submit"
              disabled={isSubmitting}
              style={{
                background: isSubmitting ? "#ff9500" : undefined,
              }}
            >
              Sign up
            </button>
          </div>
        </form>
      </div>
    </main>
  )
}
