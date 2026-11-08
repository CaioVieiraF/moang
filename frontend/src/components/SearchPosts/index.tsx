import { useEffect } from 'react'
import { Search } from 'react-feather'
import { SearchButton, SearchInput, SearchPostsContainer } from './styles'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'

const SearchFormSchema = z.object({
  query: z.string(),
})

type SearchFormSchemaType = z.infer<typeof SearchFormSchema>

interface SearchPostsProps {
  onSearch: (query: string) => void
}

/** Filters the posts the list already loaded, so no extra endpoint is needed. */
export function SearchPosts({ onSearch }: SearchPostsProps) {
  const { register, watch } = useForm<SearchFormSchemaType>({
    resolver: zodResolver(SearchFormSchema),
    defaultValues: { query: '' },
  })
  const query = watch('query') ?? ''

  useEffect(() => {
    onSearch(query)
  }, [query, onSearch])

  return (
    <SearchPostsContainer onSubmit={event => event.preventDefault()} role="search">
      <SearchInput
        type="search"
        placeholder="Buscar um post..."
        aria-label="Buscar um post"
        {...register('query')}
      />
      <SearchButton type="submit" aria-label="Buscar">
        <Search size={20} /> <span className="label">Buscar</span>
      </SearchButton>
    </SearchPostsContainer>
  )
}
