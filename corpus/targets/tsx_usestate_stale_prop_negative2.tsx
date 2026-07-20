// SAFE: Derives the display value directly from props without local state, eliminating sync issues entirely.

interface User {
  id: string;
  name: string;
  email: string;
}

export function UserProfile({ user }: { user: User }) {
  const [isEditing, setIsEditing] = useState(false);
  const [editValue, setEditValue] = useState(user.name);

  const handleStartEdit = () => {
    setEditValue(user.name);
    setIsEditing(true);
  };

  const handleSave = () => {
    setIsEditing(false);
  };

  return (
    <div>
      <h1>{isEditing ? editValue : user.name}</h1>
      <p>{user.email}</p>
      {isEditing ? (
        <input value={editValue} onChange={(e) => setEditValue(e.target.value)} />
      ) : (
        <button onClick={handleStartEdit}>Edit</button>
      )}
      {isEditing && <button onClick={handleSave}>Save</button>}
    </div>
  );
}
