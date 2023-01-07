#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

namespace space_age {
    class space_age {
        private:
        int m_age;

        public:
        space_age(const int& age) {
            this->m_age = age;
        };

        int seconds() const;
    };

}  // namespace space_age

#endif // SPACE_AGE_H