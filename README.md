# README

## Fitness-Tracker-App

### Projektbeschreibung

Diese Fitness-Tracker-App ist ein Beispielprojekt, das die Prinzipien von Domain-Driven Design (DDD) und eine saubere modulare Architektur implementiert. Die App ermöglicht Benutzern, ihre Trainingsaktivitäten zu verwalten, Trainingspläne zu erstellen und sich sozial mit anderen Benutzern zu vernetzen. Das Projekt ist in Rust geschrieben und verwendet mehrere Bounded Contexts, um die Domäne klar zu trennen und zu organisieren.

### Domain-Driven Design (DDD) Struktur

Das Projekt ist in verschiedene Bounded Contexts unterteilt, um die Verantwortlichkeiten klar zu trennen und die Wartbarkeit zu verbessern. Jeder Kontext hat seine eigenen Entitäten, Use Cases und Domänenereignisse.

### Bounded Contexts und Use Cases

1. **User Management Context**
    - **Entitäten**: User, Follow
    - **Use Cases**:
        - Registrierung und Authentifizierung
        - Profilverwaltung
        - Rollenverwaltung
        - Follow-Beziehungen

2. **Exercise Management Context**
    - **Entitäten**: Exercise
    - **Use Cases**:
        - Erstellung einer Übung
        - Bearbeitung einer Übung

3. **Training Plan Context**
    - **Entitäten**: TrainingPlan, PlanExercise
    - **Use Cases**:
        - Erstellung eines Trainingsplans
        - Zuordnung von Übungen zu Trainingsplänen
        - Beschränkung der Anzahl von Trainingsplänen

4. **Training Session Context**
    - **Entitäten**: TrainingSession
    - **Use Cases**:
        - Aufzeichnung einer Trainingseinheit
        - Anzeigen des Trainingsverlaufs

5. **Messaging and Social Context**
    - **Entitäten**: Message
    - **Use Cases**:
        - Nachricht senden
        - Nachricht empfangen

6. **Billing Context**
    - **Entitäten**: Subscription, Payment
    - **Use Cases**:
        - Abonnement erstellen
        - Zahlung verarbeiten
        - Abonnement kündigen
        - Abonnement erneuern

### Projektstruktur

```plaintext
fitness-tracker-app/
├── user_management/
│   ├── entities/
│   ├── use_cases/
│   └── events/
├── exercise_management/
│   ├── entities/
│   ├── use_cases/
│   └── events/
├── training_plan/
│   ├── entities/
│   ├── use_cases/
│   └── events/
├── training_session/
│   ├── entities/
│   ├── use_cases/
│   └── events/
├── messaging_social/
│   ├── entities/
│   ├── use_cases/
│   └── events/
├── billing/
│   ├── entities/
│   ├── use_cases/
│   └── events/
└── README.md
```

### Anforderungen

- Rust
- Cargo (Rust package manager)

### Installation

1. **Rust installieren**: [Rust installieren](https://www.rust-lang.org/tools/install)
2. **Repository klonen**:
    ```sh
    git clone https://github.com/username/fitness-tracker-app.git
    cd fitness-tracker-app
    ```

### Kompilierung und Ausführung

Um das Projekt zu kompilieren und auszuführen, verwenden Sie den folgenden Befehl:

```sh
cargo run
```

### ERD (Entity-Relationship-Diagramm)

```plaintext
User
- user_id (PK)
- username
- email
- password
- account_type (Enum: Free, Monthly, Lifetime)
- lifetime_flag (Boolean)
- join_date

Exercise
- exercise_id (PK)
- user_id (FK, User)
- name
- description
- category
- creation_date

TrainingPlan
- plan_id (PK)
- user_id (FK, User)
- name
- description
- creation_date

PlanExercise
- plan_exercise_id (PK)
- plan_id (FK, TrainingPlan)
- exercise_id (FK, Exercise)
- order (int)

TrainingSession
- session_id (PK)
- user_id (FK, User)
- plan_id (FK, TrainingPlan)
- date
- duration
- notes

Message
- message_id (PK)
- sender_id (FK, User)
- receiver_id (FK, User)
- content
- timestamp

Follow
- follow_id (PK)
- follower_id (FK, User)
- followee_id (FK, User)

Subscription
- subscription_id (PK)
- user_id (FK, User)
- plan_type (Enum: Free, Monthly, Lifetime)
- start_date
- end_date
- status (Enum: Active, Inactive, Canceled)

Payment
- payment_id (PK)
- user_id (FK, User)
- subscription_id (FK, Subscription)
- amount
- payment_date
- payment_status (Enum: Pending, Completed, Failed)
```

### Todo-Liste für Entwickler

1. **Domain Modul erstellen**:
    - Definiere Entitäten für jeden Kontext.
    - Implementiere grundlegende CRUD-Operationen.

2. **Use Cases implementieren**:
    - Definiere und implementiere Use Cases für jeden Kontext.
    - Teste die Use Cases mit Unit Tests.

3. **Domänenereignisse erstellen**:
    - Definiere und implementiere Domänenereignisse.
    - Stelle sicher, dass Ereignisse bei den entsprechenden Aktionen ausgelöst werden.

4. **DDD-Prinzipien befolgen**:
    - Trenne die Domäne klar in verschiedene Bounded Contexts.
    - Implementiere Aggregates und Repositories.

5. **Integrationstests**:
    - Schreibe Integrationstests, um die Zusammenarbeit der verschiedenen Kontexte zu testen.

6. **Dokumentation**:
    - Dokumentiere alle Module, Use Cases und Ereignisse.
    - Stelle sicher, dass die README auf dem neuesten Stand ist.

### Contributing

Beiträge sind willkommen! Bitte öffnen Sie ein Issue, bevor Sie größere Änderungen vornehmen, und stellen Sie sicher, dass alle Tests bestehen.

### Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert. Weitere Informationen finden Sie in der [LICENSE](./LICENSE)-Datei.