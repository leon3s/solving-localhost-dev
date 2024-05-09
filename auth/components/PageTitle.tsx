import React from "react"
import Image from "next/image"

export type TitleProps = {
  title: string
}

export default function PageTitle(props: TitleProps) {
  return (
    <>
      <div className="relative flex flex-col place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[140px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[--ifm-color-primary] after:dark:opacity-40 before:lg:h-[360px] p-[32px]">
        <Image src="/logo.png" alt="Next Hat logo" width={68} height={68} />
      </div>
      <div className="relative z-50">
        <h1 className="mb-[16px] text-2xl font-bold text-center">
          {props.title}
        </h1>
      </div>
    </>
  )
}
