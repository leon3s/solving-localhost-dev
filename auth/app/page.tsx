"use client"
import React from "react"
import Link from "next/link"
import { redirect } from "next/navigation"

import axios from "@/lib/api"

import PageTitle from "@/components/PageTitle"
import { useRouter, useParams, useSearchParams } from "next/navigation"

export default function Auth() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [error, setError] = React.useState("")

  const redirectUrl = searchParams.get("redirect") as string
  if (!redirectUrl) {
    redirect(process.env.DEFAULT_REDIRECT as string)
  }

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const email = (e.currentTarget.elements[0] as HTMLInputElement).value
    const password = (e.currentTarget.elements[1] as HTMLInputElement).value
    axios
      .post("/users/login", {
        key: email,
        password,
      })
      .then((_) => {
        router.replace(redirectUrl)
      })
      .catch((err) => {
        const res = err.response
        setError(
          res?.data?.msg ||
            res?.data ||
            err.message ||
            "An unknown error occurred.",
        )
      })
  }

  const signupUrl = `/signup?redirect=${redirectUrl}`

  return (
    <main className="flex flex-col items-center justify-between">
      <PageTitle title="Sign in to NextHat" />
      <div className="w-[342px]">
        <form
          onSubmit={onSubmit}
          className="text-center flex flex-col bg-[#1b1b1d] p-[16px] rounded-[6px]"
        >
          <div className="relative z-50">
            <div className="mb-4">
              <label
                className="text-left block mb-2 text-[14px]"
                htmlFor="email"
              >
                Username or email address
              </label>
              <input
                className="outline-0 w-full border border-[#30363d] rounded-[6px] bg-black py-[5px] px-[12px] focus:border-[var(--ifm-color-primary)] focus:outline-none focus:ring-1 focus:ring-[var(--ifm-color-primary)] text-[14px]"
                id="key"
              />
            </div>
            <div className="mb-4">
              <label
                className="relative text-left block mb-2 text-[14px]"
                htmlFor="password"
              >
                Password
                <Link href="/password_reset" legacyBehavior>
                  <a className="absolute right-0 bottom-0 text-[12px] text-orange-500 hover:text-orange-700">
                    Forgot password ?
                  </a>
                </Link>
              </label>
              <input
                className="outline-0 w-full border border-[#30363d] rounded-[6px] bg-black py-[5px] px-[12px] focus:border-[var(--ifm-color-primary)] focus:outline-none focus:ring-1 focus:ring-[var(--ifm-color-primary)] text-[14px]"
                id="password"
                type="password"
              />
            </div>
            <button
              className="w-full bg-orange-500 hover:bg-orange-700 text-white text-[14px] py-2 px-4 rounded-[6px] py-[5px] px-[16px] font-medium"
              type="submit"
            >
              Sign in
            </button>
          </div>
        </form>
        <p className="border border-[#30363d] rounded-[6px] p-[16px] mt-[16px] mb-[10px] text-[14px] text-center">
          New to NextHat ?{" "}
          <Link href={signupUrl} legacyBehavior>
            <a className="text-orange-500 hover:text-orange-700">
              Create an account
            </a>
          </Link>
          .
        </p>
      </div>
    </main>
  )
}
