import { format, compareDesc } from "date-fns";
import { CalendarIcon, PersonIcon } from '@radix-ui/react-icons'

import { Group } from "~/lib/data";
import { Card, CardHeader, CardTitle, CardDescription } from "~/components/ui/card";

export interface GroupCardProps {
  group: Group
}

export function GroupCard(props: GroupCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          {props.group.name}
        </CardTitle>
        <CardDescription>
          <span className="inline-flex items-baseline mr-2">
            <CalendarIcon className="self-center w-4 h-4 mx-1" />
            <span>{format(props.group.created_at, "yyyy/MM/dd")}</span>
          </span>
          <span className="inline-flex items-baseline">
            <PersonIcon className="self-center w-4 h-4 mx-1" />
            <span>{props.group.users.length}人</span>
          </span>
        </CardDescription>
      </CardHeader>
    </Card>
  )
}

export interface GroupCardListProps {
  groups: Group[]
}

export function GroupCardList(props: GroupCardListProps) {
  return (
    <div className="flex flex-col space-y-2">
      {
        props.groups
          .sort((a, b) => compareDesc(a.created_at, b.created_at))
          .map((group) => <GroupCard key={group.id} group={group} />)
      }
    </div>
  )
}
