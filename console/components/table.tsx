import React from "react"

export type TableColumn = {
  name: string
  key: string
  render?: null | ((data: any, i: number) => React.ReactNode)
}

export type TableProps = {
  data: any[]
  identifier: string | ((data: any) => string)
  columns: TableColumn[]
}

function renderHeader(columns: TableColumn[]) {
  return (
    <thead>
      <tr>
        {columns.map((column) => (
          <th
            className="border-grey-200 border-b p-2 text-left font-medium"
            key={column.key}
          >
            {column.name}
          </th>
        ))}
      </tr>
    </thead>
  )
}

function renderBody(props: TableProps) {
  return (
    <tbody>
      {props.data.map((row, i) => (
        <tr
          className="duration-2 rounded transition hover:border-[rgba(255,255,255,0.1)] hover:bg-[rgba(255,255,255,0.1)]"
          key={
            typeof props.identifier === "string"
              ? row[props.identifier]
              : props.identifier(row)
          }
        >
          {props.columns.map((column) => (
            <td className="justify-center p-2" key={column.key}>
              {column.render ? column.render(row, i) : row[column.key]}
            </td>
          ))}
        </tr>
      ))}
    </tbody>
  )
}

export function Table(props: TableProps) {
  return (
    <table className="w-full table-auto">
      {renderHeader(props.columns)}
      {renderBody(props)}
    </table>
  )
}
