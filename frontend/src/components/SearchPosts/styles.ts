import styled from 'styled-components'

export const SearchPostsContainer = styled.form`
  display: flex;
  gap: 1rem;
  margin: 1.5rem auto;

  input {
    flex: 1;
    border-radius: 3px;
    border: 0;
    border: 1px solid ${props => props.theme['primary-700']};
    background: ${props => props.theme['primary-900']};
    color: ${props => props.theme['text-300']};
    padding: 0.5rem;

    &::placeholder {
      color: ${props => props.theme['text-700']};
    }

    &:disabled {
      cursor: not-allowed;
    }
  }

  button {
    display: flex;
    align-items: center;
    gap: 0.75rem;

    border: 0;
    padding: 0.5rem;
    background: transparent;
    border: 1px solid ${props => props.theme['primary-300']};
    color: ${props => props.theme['text-300']};
    font-weight: bold;
    border-radius: 3px;
    cursor: pointer;

    &:disabled {
      color: ${props => props.theme['text-700']};
      border-color: ${props => props.theme['primary-700']};
      cursor: not-allowed;
    }

    &:not(:disabled):hover {
      background: ${props => props.theme['primary-300']};
      color: ${props => props.theme['text-100']};
    }
  }
`
