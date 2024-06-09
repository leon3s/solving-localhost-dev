"use client"

import React from "react"
import { useAppSelector } from "@/store"
import moment from "moment"

import axios from "@/lib/api"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Table } from "@/components/table"

interface DialogUserInviteProps {
  onInviteUser: (user: any) => void
}

function DialogUserInvite(props: DialogUserInviteProps) {
  const [isModalOpen, setIsModalOpen] = React.useState(false)
  const [isSubmitting, setIsSubmitting] = React.useState(false)
  const [error, setError] = React.useState(null)

  function onOpenChange() {
    setError(null)
    setIsModalOpen(!isModalOpen)
  }

  function onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault()
    const name = (e.currentTarget.elements[0] as HTMLInputElement).value
    const email = (e.currentTarget.elements[1] as HTMLInputElement).value
    const password = (e.currentTarget.elements[2] as HTMLInputElement).value

    setIsSubmitting(true)
    axios
      .post("/users", {
        name,
        email,
        password,
      })
      .then((res) => {
        setIsSubmitting(false)
        setError(null)
        onOpenChange()
        props.onInviteUser(res.data)
      })
      .catch((err) => {
        const res = err.response
        setError(
          res?.data?.msg ||
            res?.data ||
            err.message ||
            "An unknown error occurred."
        )
        setIsSubmitting(false)
      })
  }
  return (
    <>
      <Button variant="outline" onClick={onOpenChange}>
        Invite
      </Button>
      <Dialog open={isModalOpen} onOpenChange={onOpenChange}>
        <DialogContent className="relative sm:max-w-[425px] lg:top-[-50px]">
          <DialogHeader>
            <DialogTitle>Invite User</DialogTitle>
            <DialogDescription>
              Invite a new user to your organization.
            </DialogDescription>
          </DialogHeader>
          <form onSubmit={onSubmit}>
            <div className="mt-2">
              <Label htmlFor="name" className="mb-[8px]">
                Name
              </Label>
              <Input id="name" className="mt-2" />
            </div>
            <div className="mt-2">
              <Label htmlFor="username" className="mb-[8px]">
                Email
              </Label>
              <Input id="email" type="email" className="mt-2" />
            </div>
            <div className="mb-4 mt-2">
              <Label htmlFor="username" className="mb-[8px]">
                Password
              </Label>
              <Input id="password" type="password" className="mt-2" />
            </div>
            {error ? (
              <div className="mb-4 mt-2 p-1">
                <p className="text-xs text-red-500">{error}</p>
              </div>
            ) : null}
            <DialogFooter>
              <div className="flex w-full">
                <Button
                  onClick={onOpenChange}
                  variant="outline"
                  type="reset"
                  className="w-full"
                  disabled={isSubmitting}
                >
                  Cancel
                </Button>
              </div>
              <div className="flex w-full">
                <Button
                  disabled={isSubmitting}
                  type="submit"
                  className="w-full"
                >
                  {isSubmitting ? "Inviting..." : "Invite"}
                </Button>
              </div>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </>
  )
}

export default function UsersPage() {
  const me = useAppSelector((state) => state.me)
  console.log(me)
  const [users, setUsers] = React.useState<any[]>([])

  React.useEffect(() => {
    axios.get("/users").then((res) => {
      setUsers(res.data)
    })
  }, [setUsers])

  function onInviteUser(user: any) {
    setUsers([user, ...users])
  }

  return (
    <section className="container grid items-center gap-6 pb-8 pt-6 md:py-10">
      <div className="flex items-center justify-between space-y-2">
        <h2 className="text-3xl font-bold tracking-tight">Users</h2>
        <div className="flex items-center space-x-2">
          <DialogUserInvite onInviteUser={onInviteUser} />
        </div>
      </div>
      <Table
        data={users}
        identifier="id"
        columns={[
          { name: "Username", key: "name" },
          { name: "Email", key: "email" },
          {
            name: "Created at",
            key: "created_at",
            render: (data) => moment(data).fromNow(),
          },
        ]}
      />
    </section>
  )
}
