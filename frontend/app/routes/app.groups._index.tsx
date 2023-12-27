import { Group, fakeGroup } from '~/lib/data';
import { GroupCardList } from '~/components/group';

export default function GroupList() {
  const groups: Group[] = [...Array(30).keys()].map(() => fakeGroup())
  return <GroupCardList groups={groups} />;
}
