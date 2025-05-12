// TODO prevent sending empty box
// TODO probably just need to hide for dismiss, don't need to re-fetch all notes

function NoteComponent(note, fetchNotes) {
  function dismiss() {
    const requestOptions = { method: "POST", }
    fetch("api/dismiss/" + note.note_id, requestOptions)
      .then(fetchNotes)
  }
  const note_classes = `note box ${note.priority}`;
  return (
    <div className={note_classes} key={note.note_id}>
      <div>
        <span className="date">{note.timestamp}</span>
        <span style={{ float: "right" }}>
          <button type="button" onClick={dismiss}>
            <i className="fa-solid fa-box-archive"></i>
          </button>
        </span>
      </div>
      <div>{note.text}</div>
    </div>
  );
}

function NoteList({notes, fetchNotes}) {
  return notes.map((note) => NoteComponent(note, fetchNotes))
}

function JotBox({fetchNotes}) {
  function postNote() {
    const textarea = document.getElementById("jot-textarea");
    const body = {
      text: textarea.value,
      priority: "LOW",
    };
    const requestOptions = {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body)
    }
    fetch("api/post", requestOptions)
      .then(fetchNotes)
      .then(textarea.value = "")
  }

  return (
    <div className="jot-textarea-container">
      <textarea
        id="jot-textarea"
        name="jot-textarea"
        rows="5"
        cols="16"
        placeholder="jot something down..."
      ></textarea>
      <button type="button" onClick={postNote}>
        <img src="static/plane.svg" alt="submit" />
      </button>
    </div>
  )
}

function App() {
  const [notes, setNotes] = React.useState([]);
  const fetchNotes = () => {
    fetch("api/notes")
      .then((res) => res.json())
      .then((data) => {
        setNotes(data)
      })
      .catch((err) => {
        console.log(err.message)
      })
  }
  React.useEffect(fetchNotes, []);

  return (
    <>
      <div id="sidebar">
        <JotBox fetchNotes={fetchNotes}/>
      </div>
      <div id="notes">
        <NoteList notes={notes} fetchNotes={fetchNotes} />
      </div>
    </>
  )
}

const container = document.getElementById("app");
const root = ReactDOM.createRoot(container);
root.render(<App />)
