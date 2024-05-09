import React from "react"

import PageTitle from "@/components/PageTitle"

export default function PasswordReset() {
  return (
    <main className="flex flex-col items-center justify-between">
      <PageTitle title="Reset your password" />
      <div className="w-[342px]">
        <form className="text-center flex flex-col bg-[#1b1b1d] p-[16px] rounded-[6px]">
          <div className="relative z-50">
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
            <button
              className="w-full bg-orange-500 hover:bg-orange-700 text-white text-[14px] py-2 px-4 rounded-[6px] py-[5px] px-[16px] font-medium"
              type="submit"
            >
              Send password reset email
            </button>
          </div>
        </form>
      </div>
    </main>
  )
}
